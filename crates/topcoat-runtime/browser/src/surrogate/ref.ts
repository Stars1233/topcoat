export class Ref<T> {
	constructor(private readonly pointee: T) {}

	deref(): T {
		return this.pointee;
	}
}
