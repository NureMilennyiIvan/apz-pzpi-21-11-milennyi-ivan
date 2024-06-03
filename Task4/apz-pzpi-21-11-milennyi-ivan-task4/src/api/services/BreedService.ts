import { Breed } from "../../models/Breed";
import { API_URL } from "../../utils/config";
import { BreedVM } from "../../viewModels/BreedVM";
import { IBreedService } from "../interfaces/IBreedService";
import axios from "axios";

export class BreedService implements IBreedService {
    private static BREED_URLS = {
        GET_ALL: `${API_URL}/breed`,
        GET_BY_ID: (id: number) => `${API_URL}/breed/${id}`,
        CREATE: `${API_URL}/breed/create`,
        DELETE: (id: number) => `${API_URL}/breed/delete/${id}`,
        UPDATE: `${API_URL}/breed/update`,
        GET_ALL_VMS: `${API_URL}/breed-vms`
    }
    async create(item: Breed): Promise<Breed> {
        const response = await axios.post<Breed>(BreedService.BREED_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(BreedService.BREED_URLS.DELETE(itemId));
    }
    async update(item: Breed): Promise<Breed> {
        const response = await axios.put<Breed>(BreedService.BREED_URLS.UPDATE, item);
        return response.data;
    }
    async getAll(): Promise<Breed[]> {
        const response = await axios.get<Breed[]>(BreedService.BREED_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<Breed | null> {
        const response = await axios.get<Breed | null>(BreedService.BREED_URLS.GET_BY_ID(id));
        return response.data;
    }
    async getAllVMs(): Promise<BreedVM[]> {
        const response = await axios.get<BreedVM[]>(BreedService.BREED_URLS.GET_ALL_VMS);
        return response.data;
    }
}