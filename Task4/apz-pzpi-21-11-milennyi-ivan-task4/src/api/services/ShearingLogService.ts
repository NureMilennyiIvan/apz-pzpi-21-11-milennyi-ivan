import axios from "axios";
import { ShearingLog } from "../../models/ShearingLog";
import { API_URL } from "../../utils/config";
import { ShearingLogVM } from "../../viewModels/ShearingLogVM";
import { IShearingLogService } from "../interfaces/IShearingLogService";

export class ShearingService implements IShearingLogService{
    private static SHEARING_LOG_URLS = {
        GET_ALL: `${API_URL}/shearing-log`,
        GET_BY_ID: (id: number) => `${API_URL}/shearing-log/${id}`,
        CREATE: `${API_URL}/shearing-log/create`,
        DELETE: (id: number) => `${API_URL}/shearing-log/delete/${id}`,
        GET_ALL_VMS_BY_SHEEP_ID: (id: number) =>  `${API_URL}/shearing-log/sheep/${id}`
    }

    async create(item: ShearingLog): Promise<ShearingLog> {
        const response = await axios.post<ShearingLog>(ShearingService.SHEARING_LOG_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(ShearingService.SHEARING_LOG_URLS.DELETE(itemId));
    }
    async update(_item: ShearingLog): Promise<ShearingLog> {
        throw new Error("Method is forbidden.");
    }
    async getAll(): Promise<ShearingLog[]> {
        const response = await axios.get<ShearingLog[]>(ShearingService.SHEARING_LOG_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<ShearingLog | null> {
        const response = await axios.get<ShearingLog | null>(ShearingService.SHEARING_LOG_URLS.GET_BY_ID(id));
        return response.data;
    }
    async getAllVMsBySheepId(id: number): Promise<ShearingLogVM[]> {
        const response = await axios.get<ShearingLogVM[]>(ShearingService.SHEARING_LOG_URLS.GET_ALL_VMS_BY_SHEEP_ID(id));
        return response.data;
    }
}