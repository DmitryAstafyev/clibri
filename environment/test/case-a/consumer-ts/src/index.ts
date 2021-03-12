import { Client, Subject, IClientSubjects } from 'fiber';
import { Consumer } from 'fiber-consumer';
import { Connection } from 'fiber-websocket';

const connection: Connection = new Connection(`ws://127.0.0.1:8080`);
const consumer: Consumer = new Consumer(connection);
consumer.connect();

