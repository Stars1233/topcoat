export class Str {
	constructor(private readonly v: string) {}

	toJSON(): string {
		return this.v;
	}

	toString(): string {
		return this.v.toString();
	}
}
