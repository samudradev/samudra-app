PRAGMA foreign_keys=off;

ALTER TABLE konsep RENAME TO konsep_old;

CREATE TABLE konsep (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    tarikh_masuk TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    lemma_id INTEGER NOT NULL,
    golongan_id TEXT,
    keterangan TEXT,
    tertib INTEGER,
    FOREIGN KEY (lemma_id) REFERENCES lemma(id) ON DELETE SET DEFAULT,
    FOREIGN KEY (golongan_id) REFERENCES golongan_kata(id) ON UPDATE CASCADE ON DELETE SET DEFAULT
);

INSERT INTO konsep SELECT * FROM konsep_old;

DROP TABLE konsep_old;

PRAGMA foreign_keys=on;

