import { useEffect, useState } from "react";
import styles from "../../../assets/css/GridComponent.module.css"
import { useTranslation } from "react-i18next";
import { FeedingLogService } from "../../../api/services/FeedingLogService";
import { FeedingLog } from "../../../models/FeedingLog";

export const FeedingLogsGrid = () =>{
    const feedingLogService = new FeedingLogService();
    const [selectedFeedingLog, setSelectedFeedingLog] = useState<FeedingLog | null>(null);
    const [feedingLogs, setFeedingLogs] = useState<FeedingLog[]>([]);
    const [trigger, setTrigger] = useState<boolean>(true);

    const {t} = useTranslation();
    
    useEffect(() => {
        const fetchFeedingLogs= async () => {
          try {
            const data = await feedingLogService.getAll();
            setFeedingLogs(data);
          } catch (error) {
            alert(error);
            setFeedingLogs([]);
          }
        };
        fetchFeedingLogs();
    }, [trigger]);

    const handleRowSelection = (feedingLog: FeedingLog) => {
        setSelectedFeedingLog(feedingLog);
    };

    const deleteFeedingLog = async (id: number) => {
        try{
            await feedingLogService.delete(id);
            setSelectedFeedingLog(null);
            setTrigger(!trigger);
        }
        catch(error){
            console.log(error);
            alert("Error");
        }
    }
  
    return (
        <div className={styles.container}>
            <table className={styles.table}>
                <thead>
                    <tr className={styles.tr}>
                        <th className={styles.th}>{t("gridBase.chooseHeader")}</th>
                        <th className={styles.th}>{t("gridBase.idHeader")}</th>
                        <th className={styles.th}>{t("feedingLogsGrid.amountHeader")}</th>
                        <th className={styles.th}>{t("feedingLogsGrid.feedIdHeader")}</th>
                        <th className={styles.th}>{t("feedingLogsGrid.sheepIdHeader")}</th>
                        <th className={styles.th}>{t("feedingLogsGrid.shepherdIdHeader")}</th>
                        <th className={styles.th}>{t("feedingLogsGrid.timestampHeader")}</th>
                    </tr>
                </thead>
                <tbody>
                    {feedingLogs.length != 0 ? (
                        feedingLogs.map((feedingLog) => (
                            <tr key={feedingLog.id} 
                                className={`${styles.tr} ${selectedFeedingLog?.id === feedingLog.id ? styles.selected : ''}`}>
                                <td className={styles.td}>
                                    <input
                                        type="radio"
                                        name="selectedUser"
                                        checked={selectedFeedingLog?.id === feedingLog.id}
                                        onChange={() => handleRowSelection(feedingLog)}
                                    />
                                </td>
                                <td className={styles.td}>{feedingLog.id}</td>
                                <td className={styles.td}>{feedingLog.amount}</td>
                                <td className={styles.td}>{feedingLog.feed_id}</td>
                                <td className={styles.td}>{feedingLog.sheep_id}</td>
                                <td className={styles.td}>{feedingLog.shepherd_id ? feedingLog.shepherd_id : "null"}</td>
                                <td className={styles.td}>{feedingLog.timestamp}</td>
                            </tr>
                        ))
                    ) : (
                        <div className={styles.error}>{t("feedingLogsGrid.notFoundHeader")}</div>
                    )}
                </tbody>
            </table>
            <div className={styles.actionButtonsContainer}>
                <button 
                    className={`${styles.actionButton} ${selectedFeedingLog?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedFeedingLog ? deleteFeedingLog(selectedFeedingLog.id!) : undefined}
                    style={{ cursor: selectedFeedingLog ? 'pointer' : 'default' }}>
                    {t("gridBase.deleteButtonText")}
                </button>
            </div>
        </div>
    );
}