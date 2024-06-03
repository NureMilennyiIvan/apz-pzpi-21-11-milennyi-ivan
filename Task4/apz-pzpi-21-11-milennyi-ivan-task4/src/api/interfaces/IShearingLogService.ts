import { ShearingLog } from "../../models/ShearingLog";
import { ShearingLogVM } from "../../viewModels/ShearingLogVM";
import { IService } from "./IService";

export interface IShearingLogService extends IService<ShearingLog, ShearingLogVM> {
    getAllVMsBySheepId(id: number): Promise<ShearingLogVM[]>;
}