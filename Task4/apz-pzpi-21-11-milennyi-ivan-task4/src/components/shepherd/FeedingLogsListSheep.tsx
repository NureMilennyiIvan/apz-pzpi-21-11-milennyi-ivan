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
        <div className={styles.list}>
            {feedingLogsVMList.length > 0 ? (feedingLogsVMList.map((feedingLog) => (
                <div key={feedingLog.id} className={styles.card}>
                    <div className={styles.cardHeader}>
                        <h2 className={styles.logId}>Запис #{feedingLog.id}</h2>
                    </div>
                    <div className={styles.cardBody}>
                            <p><strong>Вівця:</strong> #{feedingLog.sheepId}</p>
                            <p><strong>Виконавець:</strong> {feedingLog.shepherdName !== null ? `${feedingLog.shepherdName} ${feedingLog.shepherdSurname}` : 'Немає даних'}</p>
                            <p><strong>Витрачено корму:</strong> {feedingLog.amount} кг</p>
                            <p><strong>Дата:</strong> {feedingLog.date}</p>
                    </div>
                </div>
            ))) : (
                <p>Немає інформації про записи годувань</p>
            )}
        </div>
    )
}