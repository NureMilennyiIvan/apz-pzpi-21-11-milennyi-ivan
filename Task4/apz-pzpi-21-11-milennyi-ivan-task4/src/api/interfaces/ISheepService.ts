import { Sheep } from "../../models/Sheep";
import { SheepVM } from "../../viewModels/SheepVM";
import { IService } from "./IService";

export interface ISheepService<DetailsViewModel> extends IService<Sheep, SheepVM> {
    getAllVMsByShepherdId(id: number): Promise<SheepVM[]>;
    getDetailsById(id: number): Promise<DetailsViewModel | null>;
    changeShepherd(sheepId: number, changeId: number | null): Promise<void>;
    changeTemperatureScanner(sheepId: number, changeId: number | null): Promise<void>;
}