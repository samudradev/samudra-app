WITH items AS (
    WITH params AS (SELECT ? as lemma, ? as kata_asing, ? as kata_asing_bahasa)
    SELECT
        lemma.nama AS lemma
    FROM lemma, params
        LEFT JOIN konsep ON konsep.lemma_id = lemma.id
        LEFT JOIN kata_asing_x_konsep as kaxk ON kaxk.konsep_id = konsep.id 
        LEFT JOIN kata_asing ON kata_asing.id = kaxk.kata_asing_id
    WHERE
        (CASE 
            WHEN params.lemma IS NOT NULL THEN lemma.nama = params.lemma
            ELSE lemma.nama IS NOT NULL 
        END) AND
        (CASE 
            WHEN params.kata_asing_bahasa IS NOT NULL THEN kata_asing.bahasa = params.kata_asing_bahasa
            ELSE kata_asing.bahasa IS NOT NULL 
        END) AND
        (CASE 
            WHEN params.kata_asing IS NOT NULL THEN kata_asing.nama = params.kata_asing
            ELSE kata_asing.nama IS NOT NULL 
        END)
    )
-- THE RESULTING TABLE THAT IS DESERIALIZED INTO STRUCT
SELECT 
    lemma.nama AS lemma,
    konsep.keterangan AS konsep, 
    golongan_kata.nama AS golongan_kata,
    cakupan.nama AS cakupan,
    kata_asing.nama AS kata_asing,
    kata_asing.bahasa AS bahasa_asing,
    lemma.id AS l_id,
    konsep.id AS k_id
FROM lemma, items
    LEFT JOIN konsep ON konsep.lemma_id = lemma.id
    LEFT JOIN golongan_kata ON konsep.golongan_id = golongan_kata.id
    LEFT JOIN cakupan_x_konsep as cxk ON cxk.konsep_id = konsep.id 
    LEFT JOIN cakupan ON cakupan.id = cxk.cakupan_id
    LEFT JOIN kata_asing_x_konsep as kaxk ON kaxk.konsep_id = konsep.id 
    LEFT JOIN kata_asing ON kata_asing.id = kaxk.kata_asing_id
WHERE
    lemma.nama = items.lemma

