-- セッションテーブル
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- メッセージテーブル
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- セッションIDでのメッセージ検索を高速化
CREATE INDEX idx_messages_session_id ON messages(session_id);

-- 作成日時順でのソート用
CREATE INDEX idx_messages_created_at ON messages(session_id, created_at);
