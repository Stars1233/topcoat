export class Ref<T> {
	constructor(private readonly pointee: T) {}

	deref(): T {
		return this.pointee;
	}

	toJSON(): unknown {
		return toJSONValue(this.pointee);
	}
}

function toJSONValue(value: unknown): unknown {
	if (value === null || typeof value !== "object") return value;

	const toJSON = (value as { toJSON?: unknown }).toJSON;
	if (typeof toJSON === "function") {
		return toJSONValue(toJSON.call(value));
	}

	if (Array.isArray(value)) {
		return value.map(toJSONValue);
	}

	if (Object.getPrototypeOf(value) === Object) {
		return Object.fromEntries(
			Object.entries(value as Record<string, unknown>).map(([key, item]) => [
				key,
				toJSONValue(item),
			]),
		);
	}

	return value;
}
