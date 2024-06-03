import { ShearingLog } from "../../models/ShearingLog";
import { ShearingLogVM } from "../../viewModels/ShearingLogVM";
import { IShearingLogService } from "../interfaces/IShearingLogService";

export class ShearingService implements IShearingLogService{
    getAllVMsBySheepId(id: number): Promise<ShearingLogVM[]> {
        throw new Error("Method not implemented.");
    }
    create(item: ShearingLog): Promise<ShearingLog> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: ShearingLog): Promise<ShearingLog> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<ShearingLog[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<ShearingLog | null> {
        throw new Error("Method not implemented.");
    }

}