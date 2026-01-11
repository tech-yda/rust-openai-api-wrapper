use crate::models::{ChatMessage, Session};
use sqlx::PgPool;
use uuid::Uuid;

/// セッション・メッセージのDB操作
#[derive(Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 新規セッションを作成
    pub async fn create_session(
        &self,
        system_prompt: Option<String>,
    ) -> Result<Session, sqlx::Error> {
        let id = Uuid::new_v4();
        let session = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (id, system_prompt)
            VALUES ($1, $2)
            RETURNING id, system_prompt, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(system_prompt)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    /// セッションをIDで取得
    pub async fn get_session(&self, id: Uuid) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            SELECT id, system_prompt, created_at, updated_at
            FROM sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    /// セッションにメッセージを追加
    pub async fn add_message(
        &self,
        session_id: Uuid,
        role: &str,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let message = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO messages (id, session_id, role, content)
            VALUES ($1, $2, $3, $4)
            RETURNING id, session_id, role, content, created_at
            "#,
        )
        .bind(id)
        .bind(session_id)
        .bind(role)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        // セッションのupdated_atを更新
        sqlx::query("UPDATE sessions SET updated_at = NOW() WHERE id = $1")
            .bind(session_id)
            .execute(&self.pool)
            .await?;

        Ok(message)
    }

    /// セッションの全メッセージを取得（時系列順）
    pub async fn get_messages(&self, session_id: Uuid) -> Result<Vec<ChatMessage>, sqlx::Error> {
        let messages = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, session_id, role, content, created_at
            FROM messages
            WHERE session_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }

    /// セッションを削除（カスケードでメッセージも削除）
    pub async fn delete_session(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
