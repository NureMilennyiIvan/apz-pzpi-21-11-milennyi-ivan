import { TemperatureScanner } from "../../models/TemperatureScanner";
import { TemperatureScannerVM } from "../../viewModels/TemperautreScannerVM";
import { IService } from "./IService";

export interface ITemperatureScannerService extends IService<TemperatureScanner, TemperatureScannerVM> {

}