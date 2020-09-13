import { IMessage } from './messages/in/message.holder';


export abstract class Protocol<TIncomeMessages> {

    public abstract getMsgRefs(): { [key: number]: TIncomeMessages };

    public getMsgClass(msg: IMessage): TIncomeMessages | Error {
        if (this.getMsgRefs()[msg.id] === undefined) {
            return new Error(`Fail to find implementation for message ID="${msg.id}"`);
        }
        return this.getMsgRefs()[msg.id];
    }

}
