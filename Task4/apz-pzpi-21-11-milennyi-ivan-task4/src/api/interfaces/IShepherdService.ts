import { Shepherd } from "../../models/Shepherd";
import { ShepherdVM } from "../../viewModels/ShepherdVM";
import { IService } from "./IService";

export interface IShepherdService extends IService<Shepherd, ShepherdVM> {
    getAllVMs(): Promise<ShepherdVM[]>;
}