import { useEffect, useState } from "react";
import { FeedingLogService } from "../../api/services/FeedingLogService";
import { FeedingLogVM } from "../../viewModels/FeedingLogVM";
import styles from "../../assets/css/FeedingLogsList.module.css";
interface IFeedingLogsListSheep{
    sheepId: number;
}

export const FeedingLogsListSheep: React.FC<IFeedingLogsListSheep> =({sheepId}) =>{
    const feedingLogService = new FeedingLogService();
    const [feedingLogsVMList, setFeedingLogsVMList] = useState<FeedingLogVM[]>([]);
    
    useEffect(() => {
        const fetchFeedSupplies = async () =>{
            try{
                const data = await feedingLogService.getAllVMsBySheepId(sheepId);
                setFeedingLogsVMList(data);
            }
            catch (error){
                alert(error);
                setFeedingLogsVMList([]);
            }
        }
        fetchFeedSupplies();
    }, []);
    return (
        <div>
            {feedingLogsVMList.length > 0 ? (feedingLogsVMList.map((feedingLog) => (
                <div key={feedingLog.id} className={styles.container}>
                    <div>
                        <h4>{feedingLog.id}</h4>
                    </div>
                    <div>
                        <h4>{feedingLog.date}</h4>
                    </div>
                    <div>
                        <h4>{feedingLog.amount}</h4>
                    </div>
                    <div>
                        <h4>{feedingLog.sheepId}</h4>
                    </div>
                    <div>
                        <h4>{feedingLog.shepherdName}</h4>
                    </div>
                    <div>
                        <h4>{feedingLog.shepherdSurname}</h4>
                    </div>
                </div>
            ))) : (
                <p></p>
            )}
        </div>
    )
}