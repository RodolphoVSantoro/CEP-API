import express from 'express';
import pg from 'pg';

const { Pool } = pg;

const app = express();
app.use(express.json());

console.log('Starting pg pool');
const pool = new Pool({
    user: 'postgres',
    host: 'localhost',
    database: 'cep',
    password: 'root',
    port: 54432,
});

app.get('/:cep', async (req, res) => {
    const { cep } = req.params;
    console.log(`GET /${cep}`);

    const zip = <T, J>(a: T[], b: J[]) => a.map((k: T, i: number) => [k, b[i]]);

    const result = await pool.query(`SELECT c.*,
    (SELECT array(SELECT f.caixa_inicial FROM faixas_caixa_postal f WHERE f.cep_id = c.id ORDER BY f.id)) AS caixa_inicial,
    (SELECT array(SELECT f.caixa_final FROM faixas_caixa_postal f WHERE f.cep_id = c.id ORDER BY f.id)) AS caixa_final
    FROM ceps as c where c.cep LIKE '${cep}%'`);
    const ceps = result.rows.map((cep) => ({
        id: cep.id,
        bairro_num: cep.bairro_num,
        cep: cep.cep,
        faixas_caixa_postal: zip(cep.caixa_final ?? [], cep.caixa_inicial ?? [])
            .map((caixas) => ({ caixa_inicial: caixas[1], caixa_final: caixas[0] })),
        faixas_cep: cep.faixas_cep,
        localidade_no_sem: cep.localidade_no_sem,
        localidade_num: cep.localidade_num,
        localidade: cep.localidade,
        localidade_subordinada: cep.localidade_subordinada,
        logradouro_dnec: cep.logradouro_dnec,
        logradouro_texto: cep.logradouro_texto,
        nome_unidade: cep.nome_unidade,
        numero_localidade: cep.numero_localidade,
        situacao: cep.situacao,
        tipo_cep: cep.tipo_cep,
        uf: cep.uf,
    }));
    res.status(200).json(ceps);
});

app.listen(3001, () => console.log('Server running on port 3001'));