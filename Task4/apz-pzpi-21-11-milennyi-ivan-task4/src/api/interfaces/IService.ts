export interface IService<Model, ViewModel> {
    create(item: Model): Promise<Model>;
    delete(itemId: number): Promise<void>;
    update(item: Model): Promise<Model>;
    getAll(): Promise<Model[]>;
    getById(id: number): Promise<Model | null>;
}