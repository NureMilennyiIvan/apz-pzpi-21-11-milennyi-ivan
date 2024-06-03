import { TemperatureScanner } from "../../models/TemperatureScanner";
import { ITemperatureScannerService } from "../interfaces/ITemperatureScannerService";

export class TemperatureScannerService implements ITemperatureScannerService{
    create(item: TemperatureScanner): Promise<TemperatureScanner> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: TemperatureScanner): Promise<TemperatureScanner> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<TemperatureScanner[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<TemperatureScanner | null> {
        throw new Error("Method not implemented.");
    }

}