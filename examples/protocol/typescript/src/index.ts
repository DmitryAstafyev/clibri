import * as Protocol from "./protocol";

function encode_and_pack() {
	const structExampleA: Protocol.StructExampleA =
		Protocol.StructExampleA.defaults();
	const encodedMessage: ArrayBufferLike = structExampleA.encode();
	const sequence: number = 1;
	const uuid: string = "some unique uuid";
	const packedMessage: ArrayBufferLike = structExampleA.pack(sequence, uuid);
	console.log(
		`encoded: ${encodedMessage.byteLength} bytes / packed: ${packedMessage.byteLength} bytes`
	);
}

function buffer_reading() {
	const buffer = Buffer.concat([
		new Uint8Array(Protocol.StructExampleA.defaults().pack(1)),
		new Uint8Array(Protocol.StructExampleB.defaults().pack(2)),
		new Uint8Array(Protocol.StructExampleC.defaults().pack(3)),
	]);
	const reader: Protocol.BufferReaderMessages =
		new Protocol.BufferReaderMessages();
	reader.chunk(buffer);
	do {
		const received:
			| Protocol.IAvailableMessage<Protocol.IAvailableMessages>
			| undefined = reader.next();
		if (received === undefined) {
			// No more messages in buffer
			break;
		}
		if (received.msg.StructExampleA !== undefined) {
			console.log(`StructExampleA has been gotten`);
		} else if (received.msg.StructExampleB !== undefined) {
			console.log(`StructExampleB has been gotten`);
		} else if (received.msg.StructExampleC !== undefined) {
			console.log(`StructExampleC has been gotten`);
		}
	} while (true);
}

encode_and_pack();
buffer_reading();
