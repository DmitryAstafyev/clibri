import { Client, Subject, IClientSubjects } from 'fiber';
import { Consumer, Protocol } from './consumer/index';
import { Connection } from 'fiber-websocket';

const connection: Connection = new Connection(`ws://127.0.0.1:8080`);
const consumer: Consumer = new Consumer(connection);
consumer.connected.subscribe(() => {
    console.log(`Consumer is connected!`);
    consumer.assign({
        id: BigInt(123),
        uuid: 'Some UUID',
        location: 'London'
    }).then((uuid: string) => {
        console.log(`UUID is gotten ${uuid}!`);
    }).catch((err: Error) => {
        console.error(err);
    });
});

