


-- Tabelle Benutzer
CREATE TABLE IF NOT EXISTS benutzer (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    benutzername TEXT NOT NULL,
    email TEXT NOT NULL,
    passwort TEXT NOT NULL
);