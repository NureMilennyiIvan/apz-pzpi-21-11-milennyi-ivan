import { Breed } from "../../models/Breed";
import { BreedVM } from "../../viewModels/BreedVM";
import { IService } from "./IService";

export interface IBreedService extends IService<Breed, BreedVM> {
    getAllVMs(): Promise<BreedVM[]>;
}