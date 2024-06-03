import { Sheep } from "../../models/Sheep";
import { SheepVM } from "../../viewModels/SheepVM";
import { SheepDetailsVM } from "../../viewModels/extraViewModels/SheepDetailsVM";
import { ISheepService } from "../interfaces/ISheepService";

export class SheepService implements ISheepService<SheepDetailsVM> {
    getAllVMsByShepherdId(id: number): Promise<SheepVM[]> {
        throw new Error("Method not implemented.");
    }
    getDetailsById(id: number): Promise<SheepDetailsVM | null> {
        throw new Error("Method not implemented.");
    }
    changeShepherd(sheepId: number, shepherdId: number | null): Promise<void> {
        throw new Error("Method not implemented.");
    }
    changeTemperatureScanner(sheepId: number, temperatureScannerId: number | null): Promise<void> {
        throw new Error("Method not implemented.");
    }
    create(item: Sheep): Promise<Sheep> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: Sheep): Promise<Sheep> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<Sheep[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<Sheep | null> {
        throw new Error("Method not implemented.");
    }

}
