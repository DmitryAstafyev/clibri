import { Client, Subject, IClientSubjects } from 'fiber';
import { Consumer, Protocol } from './consumer/index';
import { Connection } from 'fiber-websocket';

const connection: Connection = new Connection(`ws://127.0.0.1:8080`);
const consumer: Consumer = new Consumer(connection, {
    id: BigInt(123),
    uuid: 'Some UUID',
    location: 'London'
});
consumer.connected.subscribe(() => {
    console.log(`Consumer is connected!`);
});
consumer.ready.subscribe(() => {
    console.log(`Consumer is ready!`);
});

