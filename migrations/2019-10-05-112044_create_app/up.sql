CREATE TABLE app (
   id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
   jwt_secret VARCHAR(32) NOT NULL
);
