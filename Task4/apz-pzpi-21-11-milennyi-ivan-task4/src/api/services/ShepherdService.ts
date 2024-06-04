import axios from "axios";
import { Shepherd } from "../../models/Shepherd";
import { API_URL } from "../../utils/config";
import { AuthService } from "../interfaces/IAuthService";
import { IShepherdService } from "../interfaces/IShepherdService";

export class ShepherdService implements IShepherdService, AuthService<Shepherd>{
    private static SHEPHERD_URLS = {
        GET_ALL: `${API_URL}/shepherd`,
        GET_BY_ID: (id: number) => `${API_URL}/shepherd/${id}`,
        CREATE: `${API_URL}/shepherd/create`,
        DELETE: (id: number) => `${API_URL}/shepherd/delete/${id}`,
        UPDATE: `${API_URL}/shepherd/update`,
        AUTHORIZE: `${API_URL}/shepherd/authorize`
    }
    async create(item: Shepherd): Promise<Shepherd> {
        const response = await axios.post<Shepherd>(ShepherdService.SHEPHERD_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(ShepherdService.SHEPHERD_URLS.DELETE(itemId));
    }
    async update(item: Shepherd): Promise<Shepherd> {
        const response = await axios.put<Shepherd>(ShepherdService.SHEPHERD_URLS.UPDATE, item);
        return response.data;
    }
    async getAll(): Promise<Shepherd[]> {
        const response = await axios.get<Shepherd[]>(ShepherdService.SHEPHERD_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<Shepherd | null> {
        const response = await axios.get<Shepherd | null>(ShepherdService.SHEPHERD_URLS.GET_BY_ID(id));
        return response.data;
    }
    async authorize(username: string, passwordHash: string): Promise<Shepherd | null> {
        console.log("authorize", username, passwordHash);
        const response = await axios.post<Shepherd | null>(ShepherdService.SHEPHERD_URLS.AUTHORIZE, {username: username, password_hash: passwordHash});
        return response.data;
    }
}
