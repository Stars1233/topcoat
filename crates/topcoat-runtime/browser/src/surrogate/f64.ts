export class F64 {
	constructor(private readonly v: number) {}

	add(other: F64): F64 {
		return new F64(this.v + other.v);
	}

	sub(other: F64): F64 {
		return new F64(this.v - other.v);
	}

	mul(other: F64): F64 {
		return new F64(this.v * other.v);
	}

	div(other: F64): F64 {
		return new F64(this.v / other.v);
	}

	toString(): string {
		return this.v.toString();
	}
}
