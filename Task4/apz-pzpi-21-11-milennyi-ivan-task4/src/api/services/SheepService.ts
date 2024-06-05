import axios from "axios";
import { Sheep } from "../../models/Sheep";
import { API_URL } from "../../utils/config";
import { SheepVM } from "../../viewModels/SheepVM";
import { SheepDetailsVM } from "../../viewModels/extraViewModels/SheepDetailsVM";
import { ISheepService } from "../interfaces/ISheepService";

export class SheepService implements ISheepService<SheepDetailsVM> {
    private static SHEEP_URLS = {
        GET_ALL: `${API_URL}/sheep`,
        GET_BY_ID: (id: number) => `${API_URL}/sheep/${id}`,
        CREATE: `${API_URL}/sheep/create`,
        DELETE: (id: number) => `${API_URL}/sheep/delete/${id}`,
        UPDATE: `${API_URL}/sheep/update`,
        GET_ALL_VMS_BY_SHEPHERD_ID: (id: number) => `${API_URL}/sheep/shepherd/${id}`,
        GET_DETAILS_BY_ID: (id: number) => `${API_URL}/sheep/details/${id}`,
        CHANGE_SHEPHERD: (sheepId: number) => `${API_URL}/sheep/change-shepherd/${sheepId}`,
        CHANGE_TEMPERATURE_SCANNER: (sheepId: number) => `${API_URL}/sheep/change-temperature-scanner/${sheepId}`
    }
    async create(item: Sheep): Promise<Sheep> {
        const response = await axios.post<Sheep>(SheepService.SHEEP_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(SheepService.SHEEP_URLS.DELETE(itemId));
    }
    async update(item: Sheep): Promise<Sheep> {
        const response = await axios.put<Sheep>(SheepService.SHEEP_URLS.UPDATE, item);
        return response.data;
    }
    async getAll(): Promise<Sheep[]> {
        const response = await axios.get<Sheep[]>(SheepService.SHEEP_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<Sheep | null> {
        const response = await axios.get<Sheep | null>(SheepService.SHEEP_URLS.GET_BY_ID(id));
        return response.data;
    }
    async getAllVMsByShepherdId(id: number): Promise<SheepVM[]> {
        const response = await axios.get<[]>(SheepService.SHEEP_URLS.GET_ALL_VMS_BY_SHEPHERD_ID(id));
        const vms: SheepVM[] = [];
        response.data.map(sheep => {
            //@ts-ignore
            vms.push(new SheepVM(sheep.id, sheep.breed, sheep.sex, sheep.birth_date, sheep.last_feeding_timestamp, sheep.last_shearing_timestamp));
        });
        return vms;
    }
    async getDetailsById(id: number): Promise<SheepDetailsVM | null> {
        const response = await axios.get(SheepService.SHEEP_URLS.GET_DETAILS_BY_ID(id));
        const sheepDetails = response.data;
        if (sheepDetails){
            //@ts-ignore
            return new SheepDetailsVM(sheepDetails.id, sheepDetails.breed, sheepDetails.breed_info, sheepDetails.sex, sheepDetails.birth_date, sheepDetails.weight, sheepDetails.feed_id, 
                sheepDetails.feed_name, sheepDetails.feed_amount, sheepDetails.last_feeding_timestamp, sheepDetails.last_shearing_timestamp, sheepDetails.temperature); 
        }
        return null;
    }
    async changeShepherd(sheepId: number, changeId: number | null): Promise<void> {
        await axios.patch<void>(SheepService.SHEEP_URLS.CHANGE_SHEPHERD(sheepId), {change_id: changeId})
    }
    async changeTemperatureScanner(sheepId: number, changeId: number | null): Promise<void> {
        await axios.patch<void>(SheepService.SHEEP_URLS.CHANGE_TEMPERATURE_SCANNER(sheepId), {change_id: changeId})
    }
}