export abstract class Base {
  static $$id = 0;
  abstract name: string;
  id: number;
  type: string;

  constructor() {
    this.id = Base.$$id++;
  }
}
