import express from 'express';
import setupExpressWs from 'express-ws';

const app = express();
const port = 3000;
setupExpressWs(app);
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(express.static('client/dist'));

(app as any).ws('/', async (ws: WebSocket, req: Express.Request) => {
  // const id = socketStore.add(ws);
  //TODO
});

app.listen(
  port, () => console.log(`Example app listening at http://localhost:${port}`)
);





