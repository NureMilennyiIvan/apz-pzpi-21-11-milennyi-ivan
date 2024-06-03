import { FeedSupply } from "../../models/FeedSupply";
import { FeedSupplyVM } from "../../viewModels/FeedSupplyVM";
import { IService } from "./IService";

export interface IFeedSupplyService extends IService<FeedSupply, FeedSupplyVM> {
    getAllVMs(): Promise<FeedSupplyVM[]>;
}