export abstract class Base {
  static $$id = 0;
  abstract name: string;
  id: number;

  constructor() {
    this.id = Base.$$id++;
  }
}
