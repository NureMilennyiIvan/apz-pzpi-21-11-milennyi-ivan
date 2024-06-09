import { useEffect, useState } from "react";
import { ShearingLogService } from "../../api/services/ShearingLogService";
import { ShearingLogVM } from "../../viewModels/ShearingLogVM";
import styles from "../../assets/css/ShearingLogsList.module.css";
import { useTranslation } from "react-i18next";
interface IShearingLogsList{
    sheepId: number;
}

export const ShearingLogsList: React.FC<IShearingLogsList> =({sheepId}) =>{
    const shearingLogService = new ShearingLogService();
    const [shearingLogsVMList, setShearingLogsVMList] = useState<ShearingLogVM[]>([]);
    
    const {t} = useTranslation();

    useEffect(() => {
        const fetchFeedSupplies = async () =>{
            try{
                const data = await shearingLogService.getAllVMsBySheepId(sheepId);
                setShearingLogsVMList(data);
            }
            catch (error){
                alert(error);
                setShearingLogsVMList([]);
            }
        }
        fetchFeedSupplies();
    }, []);
    return (
        <div className={styles.list}>
            {shearingLogsVMList.length > 0 ? (shearingLogsVMList.map((shearingLog) => (
                <div key={shearingLog.id} className={styles.card}>
                    <div className={styles.cardHeader}>
                        <h2 className={styles.logId}>{t("shearingLogsList.logHeader")} #{shearingLog.id}</h2>
                    </div>
                    <div className={styles.cardBody}>
                            <p><strong>{t("shearingLogsList.sheepHeader")}:</strong> #{shearingLog.sheepId}</p>
                            <p><strong>{t("shearingLogsList.performerHeader")}:</strong> {shearingLog.shepherdName !== null ? `${shearingLog.shepherdName} ${shearingLog.shepherdSurname}` : `${t("shearingLogsList.noDataHeader")}`}</p>
                            <p><strong>{t("shearingLogsList.feedAmountHeader")}:</strong> {shearingLog.woolAmount} {t("shearingLogsList.feedAmountUnitsHeader")}</p>
                            <p><strong>{t("shearingLogsList.dateHeader")}:</strong> {shearingLog.date}</p>
                    </div>
                </div>
            ))) : (
                <p>{t("shearingLogsList.notFoundHeader")}</p>
            )}
        </div>
    )
}