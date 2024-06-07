import { useEffect, useState } from "react";
import { ShearingLogService } from "../../api/services/ShearingLogService";
import { ShearingLogVM } from "../../viewModels/ShearingLogVM";

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
        <div>
            {shearingLogsVMList.length > 0 ? (shearingLogsVMList.map((shearingLog) => (
                <div key={shearingLog.id}>
                    <div>
                        <h4>{shearingLog.id}</h4>
                    </div>
                    <div>
                        <h4>{shearingLog.date}</h4>
                    </div>
                    <div>
                        <h4>{shearingLog.woolAmount}</h4>
                    </div>
                    <div>
                        <h4>{shearingLog.sheepId}</h4>
                    </div>
                    <div>
                        <h4>{shearingLog.shepherdName}</h4>
                    </div>
                    <div>
                        <h4>{shearingLog.shepherdSurname}</h4>
                    </div>
                </div>
            ))) : (
                <p></p>
            )}
        </div>
    )
}