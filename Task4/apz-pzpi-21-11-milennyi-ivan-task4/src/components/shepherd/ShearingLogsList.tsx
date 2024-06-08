import { useEffect, useState } from "react";
import { ShearingLogService } from "../../api/services/ShearingLogService";
import { ShearingLogVM } from "../../viewModels/ShearingLogVM";
import styles from "../../assets/css/ShearingLogsList.module.css";
interface IShearingLogsList{
    sheepId: number;
}

export const ShearingLogsList: React.FC<IShearingLogsList> =({sheepId}) =>{
    const shearingLogService = new ShearingLogService();
    const [shearingLogsVMList, setShearingLogsVMList] = useState<ShearingLogVM[]>([]);
    
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
                        <h2 className={styles.logId}>Запис #{shearingLog.id}</h2>
                    </div>
                    <div className={styles.cardBody}>
                            <p><strong>Вівця:</strong> #{shearingLog.sheepId}</p>
                            <p><strong>Виконавець:</strong> {shearingLog.shepherdName !== null ? `${shearingLog.shepherdName} ${shearingLog.shepherdSurname}` : 'Немає даних'}</p>
                            <p><strong>Отримано шерсті:</strong> {shearingLog.woolAmount} кг</p>
                            <p><strong>Дата:</strong> {shearingLog.date}</p>
                    </div>
                </div>
            ))) : (
                <p>Немає інформації про записи стрижок</p>
            )}
        </div>
    )
}