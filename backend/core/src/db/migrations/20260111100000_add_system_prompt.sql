-- セッションにシステムプロンプトを追加
ALTER TABLE sessions ADD COLUMN system_prompt TEXT;
