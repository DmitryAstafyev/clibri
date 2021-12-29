import {
	Consumer,
	Protocol,
	BeaconBeaconA,
	BeaconBeaconsBeaconA,
	BeaconBeaconsBeaconB,
	BeaconBeaconsSubBeaconA,
} from "./consumer/index";
import { Connection } from "clibri-websocket-client-node";
import { Test } from "./test";
import { Subscription, Subject } from "clibri";
import { Stat, Alias } from "./stat";
import { panic } from "./tools";

export class Client {
	private _connection!: Connection;
	private _consumer!: Consumer;
	private _subscriptions: { [key: string]: Subscription } = {};
	private _stat: Stat = new Stat();
	private _destroyed: boolean = false;

	public done: Subject<Stat> = new Subject();

	constructor(addr: string) {
		this._connection = new Connection(addr);
		this._consumer = new Consumer(
			this._connection,
			{
				field_bool: true,
				field_f32: 0.1,
				field_f64: 0.2,
				field_i8: -1,
				field_i16: -2,
				field_i32: -3,
				field_i64: -BigInt(4),
				field_u8: 1,
				field_u16: 2,
				field_u32: 3,
				field_u64: BigInt(4),
				field_str: "",
				field_str_empty: "",
			},
			{ global: false }
		);
		//this._consumer.connected.subscribe(this._onConnected.bind(this));
		this._consumer.disconnected.subscribe(this._onDisconnected.bind(this));
		this._consumer.ready.subscribe(this._onReady.bind(this));
		this._subscriptions.StructA =
			this._consumer.broadcast.StructA.subscribe(() => {
				this._stat.case(Alias.StructA);
			});
		this._subscriptions.StructB =
			this._consumer.broadcast.StructB.subscribe(() => {
				this._stat.case(Alias.StructB);
			});
		this._subscriptions.StructC =
			this._consumer.broadcast.StructC.subscribe(() => {
				this._stat.case(Alias.StructC);
			});
		this._subscriptions.StructD =
			this._consumer.broadcast.StructD.subscribe(() => {
				this._stat.case(Alias.StructD);
			});
		this._subscriptions.StructF =
			this._consumer.broadcast.StructF.subscribe(() => {
				this._stat.case(Alias.StructF);
			});
		this._subscriptions.StructJ =
			this._consumer.broadcast.StructJ.subscribe(() => {
				this._stat.case(Alias.StructJ);
			});
		this._subscriptions.GroupBGroupCStructB =
			this._consumer.broadcast.GroupBGroupCStructB.subscribe(() => {
				this._stat.case(Alias.GroupBGroupCStructB);
			});
		this._subscriptions.GroupAStructA =
			this._consumer.broadcast.GroupAStructA.subscribe(() => {
				this._stat.case(Alias.GroupAStructA);
			});
		this._subscriptions.GroupAStructB =
			this._consumer.broadcast.GroupAStructB.subscribe(() => {
				this._stat.case(Alias.GroupAStructB);
			});
		this._subscriptions.GroupBStructA =
			this._consumer.broadcast.GroupBStructA.subscribe(() => {
				this._stat.case(Alias.GroupBStructA);
			});
		this._subscriptions.GroupBGroupCStructA =
			this._consumer.broadcast.GroupBGroupCStructA.subscribe(() => {
				this._stat.case(Alias.GroupBGroupCStructA);
			});
		this._subscriptions.Connected = this._consumer.connected.subscribe(
			() => {
				this._stat.case(Alias.Connected);
			}
		);
		this._subscriptions.Disconnected =
			this._consumer.disconnected.subscribe(() => {
				this._stat.case(Alias.Disconnected);
			});
		this._subscriptions.Error = this._consumer.error.subscribe(() => {
			this._stat.case(Alias.Error);
		});
		this._subscriptions.TriggerBeacons =
			this._consumer.broadcast.TriggerBeacons.subscribe(() => {
				this._stat.case(Alias.TriggerBeacons);
				Promise.allSettled([
					new BeaconBeaconA(
						Protocol.BeaconA.defaults(),
						this._consumer
					).send(),
					new BeaconBeaconsBeaconA(
						Protocol.Beacons.BeaconA.defaults(),
						this._consumer
					).send(),
					new BeaconBeaconsBeaconB(
						Protocol.Beacons.BeaconB.defaults(),
						this._consumer
					).send(),
					new BeaconBeaconsSubBeaconA(
						Protocol.Beacons.Sub.BeaconA.defaults(),
						this._consumer
					).send(),
				])
					.then(() => {
						this._stat.case(Alias.BeaconA);
						this._stat.case(Alias.BeaconsBeaconA);
						this._stat.case(Alias.BeaconsBeaconB);
						this._stat.case(Alias.BeaconsSubBeaconA);
						this._stat.beacons();
					})
					.catch((error: Error) => {
						panic(`Fail to send beacons: ${error.message}`);
					});
			});
		this._subscriptions.FinishConsumerTestBroadcast =
			this._consumer.broadcast.FinishConsumerTestBroadcast.subscribe(
				() => {
					this._stat.case(Alias.FinishConsumerTestBroadcast);
					this._stat.finish();
				}
			);
	}

	private _onDisconnected() {
		this._drop();
	}

	private _drop() {
		if (this._destroyed) {
			panic(`Client cannot be destroyed multiple times`);
		}
		Object.keys(this._subscriptions).forEach((sub: string) => {
			this._subscriptions[sub].destroy();
		});
		this._stat.case(Alias.Disconnected);
		this.done.emit(this._stat);
		this._destroyed = true;
	}

	private _onReady() {
		const test = new Test();
		test.run(this._consumer).catch((error: Error) => {
			panic(error.message);
		});
		this._stat.onFinish(() => {
			this._consumer
				.destroy()
				.then(() => {
					this._drop();
				})
				.catch((error: Error) => {
					panic(error.message);
				});
		});
	}
}
