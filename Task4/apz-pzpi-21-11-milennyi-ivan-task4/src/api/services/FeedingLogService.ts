import axios from "axios";
import { FeedingLog } from "../../models/FeedingLog";
import { API_URL } from "../../utils/config";
import { FeedingLogVM } from "../../viewModels/FeedingLogVM";
import { IFeedingLogService } from "../interfaces/IFeedingLogService";

export class FeedingLogService implements IFeedingLogService {
    private static FEEDING_LOG_URLS = {
        GET_ALL: `${API_URL}/feeding-log`,
        GET_BY_ID: (id: number) => `${API_URL}/feeding-log/${id}`,
        CREATE: `${API_URL}/feeding-log/create`,
        DELETE: (id: number) => `${API_URL}/feeding-log/delete/${id}`,
        GET_ALL_VMS_BY_SHEEP_ID: (id: number) =>  `${API_URL}/feeding-log/sheep/${id}`,
        GET_ALL_VMS_BY_FEED_ID: (id: number) =>  `${API_URL}/feeding-log/feed/${id}`
    }
    async create(item: FeedingLog): Promise<FeedingLog> {
        const response = await axios.post<FeedingLog>(FeedingLogService.FEEDING_LOG_URLS.CREATE, item);
        return response.data;
    }
    async delete(itemId: number): Promise<void> {
        await axios.delete(FeedingLogService.FEEDING_LOG_URLS.DELETE(itemId));
    }
    async update(_item: FeedingLog): Promise<FeedingLog> {
        throw new Error("Method is forbidden.");
    }
    async getAll(): Promise<FeedingLog[]> {
        const response = await axios.get<FeedingLog[]>(FeedingLogService.FEEDING_LOG_URLS.GET_ALL);
        return response.data;
    }
    async getById(id: number): Promise<FeedingLog | null> {
        const response = await axios.get<FeedingLog | null>(FeedingLogService.FEEDING_LOG_URLS.GET_BY_ID(id));
        return response.data;
    }
    async getAllVMsBySheepId(id: number): Promise<FeedingLogVM[]> {
        const response = await axios.get<[]>(FeedingLogService.FEEDING_LOG_URLS.GET_ALL_VMS_BY_SHEEP_ID(id));
        const vms: FeedingLogVM[] = [];
        response.data.map(feedingLog => {
            //@ts-ignore
            vms.push(new FeedingLogVM(feedingLog.id, feedingLog.timestamp, feedingLog.amount, feedingLog.sheep_id, feedingLog.shepherd_name, feedingLog.shepherd_surname));
        });
        return vms;
    }
    async getAllVMsByFeedId(id: number): Promise<FeedingLogVM[]> {
        const response = await axios.get<[]>(FeedingLogService.FEEDING_LOG_URLS.GET_ALL_VMS_BY_FEED_ID(id));
        const vms: FeedingLogVM[] = [];
        response.data.map(feedingLog => {
            //@ts-ignore
            vms.push(new FeedingLogVM(feedingLog.id, feedingLog.timestamp, feedingLog.amount, feedingLog.sheep_id, feedingLog.shepherd_name, feedingLog.shepherd_surname));
        });
        return vms;
    }
}