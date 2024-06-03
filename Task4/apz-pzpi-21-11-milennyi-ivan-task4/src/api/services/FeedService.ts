import axios from "axios";
import { Feed } from "../../models/Feed";
import { API_URL } from "../../utils/config";
import { FeedVM } from "../../viewModels/FeedVM";
import { IFeedService } from "../interfaces/IFeedServiceService";

export class FeedService implements IFeedService {
    private static FEED_URLS = {
        GET_ALL: `${API_URL}/feed`,
        GET_BY_ID: (id: number) => `${API_URL}/feed/${id}`,
        CREATE: `${API_URL}/feed/create`,
        DELETE: (id: number) => `${API_URL}/feed/delete/${id}`,
        UPDATE: `${API_URL}/feed/update`,
        GET_ALL_VMS: `${API_URL}/feed-vms`
    }
    async create(item: Feed): Promise<Feed> {
        const response = await axios.post<Feed>(FeedService.FEED_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(FeedService.FEED_URLS.DELETE(itemId));
    }
    async update(item: Feed): Promise<Feed> {
        const response = await axios.put<Feed>(FeedService.FEED_URLS.UPDATE, item);
        return response.data;
    }
    async getAll(): Promise<Feed[]> {
        const response = await axios.get<Feed[]>(FeedService.FEED_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<Feed | null> {
        const response = await axios.get<Feed | null>(FeedService.FEED_URLS.GET_BY_ID(id));
        return response.data;
    }
    async getAllVMs(): Promise<FeedVM[]> {
        const response = await axios.get<FeedVM[]>(FeedService.FEED_URLS.GET_ALL_VMS);
        return response.data;
    }
}