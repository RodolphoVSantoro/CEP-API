import mongoose from "mongoose";
import express from "express";

const mongoConnection = mongoose.connect("mongodb://localhost:27017/CEP");
const CEP = mongoose.model('cep', new mongoose.Schema(Object));
const app = express();

app.use(express.json());

app.get('/cep/:cep', async (req, res) => {
    const { cep } = req.params;
    console.log(cep);
    const result = await CEP.find({ cep: new RegExp(cep) });
    res.json(result);
});

app.listen(3000, () => console.log('Server running on port 3000'));
