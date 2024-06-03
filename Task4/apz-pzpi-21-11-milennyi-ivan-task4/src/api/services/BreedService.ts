import { Breed } from "../../models/Breed";
import { BreedVM } from "../../viewModels/BreedVM";
import { IBreedService } from "../interfaces/IBreedService";

export class BreedService implements IBreedService {
    getAllVMs(): Promise<BreedVM[]> {
        throw new Error("Method not implemented.");
    }
    create(item: Breed): Promise<Breed> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: Breed): Promise<Breed> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<Breed[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<Breed | null> {
        throw new Error("Method not implemented.");
    }
}