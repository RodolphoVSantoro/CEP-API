-- Add migration script here
CREATE TABLE ceps (
  id SERIAL PRIMARY KEY,
  cep VARCHAR(8) NOT NULL,
  faixas_cep TEXT[] NOT NULL,
  bairro_num VARCHAR(255) NOT NULL,
  localidade_no_sem VARCHAR(255) NOT NULL,
  localidade_num VARCHAR(255) NOT NULL,
  localidade VARCHAR(255) NOT NULL,
  localidade_subordinada VARCHAR(255) NOT NULL,
  logradouro_dnec VARCHAR(255) NOT NULL,
  logradouro_texto VARCHAR(255) NOT NULL,
  nome_unidade VARCHAR(255) NOT NULL,
  numero_localidade VARCHAR(255) NOT NULL,
  situacao VARCHAR(255) NOT NULL,
  tipo_cep VARCHAR(255) NOT NULL,
  uf VARCHAR(255) NOT NULL
);

CREATE TABLE faixas_caixa_postal (
  id SERIAL PRIMARY KEY,
  cep_id INTEGER NOT NULL,
  caixa_inicial VARCHAR(255),
  caixa_final VARCHAR(255),
  CONSTRAINT faixas_caixa_postal_ceps_id_fk FOREIGN KEY (cep_id) REFERENCES ceps (id)
);

CREATE UNIQUE INDEX ceps_cep_uindex ON ceps (cep);
