DO $$ 
BEGIN 
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'messages' AND column_name = 'author') THEN
        ALTER TABLE messages ADD COLUMN author VARCHAR(255) NOT NULL DEFAULT 'Anonymous';
    END IF;
END $$;

-- Update existing messages with Reddit-style usernames
UPDATE messages SET author = 'GameMaster42' WHERE id = 1 AND author = 'Anonymous';
UPDATE messages SET author = 'PixelWarrior_99' WHERE id = 2 AND author = 'Anonymous'; 