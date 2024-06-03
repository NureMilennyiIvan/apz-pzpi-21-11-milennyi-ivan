import axios from "axios";
import { TemperatureScanner } from "../../models/TemperatureScanner";
import { API_URL } from "../../utils/config";
import { ITemperatureScannerService } from "../interfaces/ITemperatureScannerService";

export class TemperatureScannerService implements ITemperatureScannerService{
    private static TEMPERATURE_SCANNER_URLS = {
        GET_ALL: `${API_URL}/temperature-scanner`,
        GET_BY_ID: (id: number) => `${API_URL}/temperature-scanner/${id}`,
        CREATE: `${API_URL}/temperature-scanner/create`,
        DELETE: (id: number) => `${API_URL}/temperature-scanner/delete/${id}`,
        UPDATE: `${API_URL}/temperature-scanner/update`
    }
    async create(item: TemperatureScanner): Promise<TemperatureScanner> {
        const response = await axios.post<TemperatureScanner>(TemperatureScannerService.TEMPERATURE_SCANNER_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(TemperatureScannerService.TEMPERATURE_SCANNER_URLS.DELETE(itemId));
    }
    async update(item: TemperatureScanner): Promise<TemperatureScanner> {
        const response = await axios.put<TemperatureScanner>(TemperatureScannerService.TEMPERATURE_SCANNER_URLS.UPDATE, item);
        return response.data;
    }
    async getAll(): Promise<TemperatureScanner[]> {
        const response = await axios.get<TemperatureScanner[]>(TemperatureScannerService.TEMPERATURE_SCANNER_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<TemperatureScanner | null> {
        const response = await axios.get<TemperatureScanner | null>(TemperatureScannerService.TEMPERATURE_SCANNER_URLS.GET_BY_ID(id));
        return response.data;
    }

}