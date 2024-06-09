import { useEffect, useState } from "react";
import { SheepService } from "../../api/services/SheepService";
import { SheepVM } from "../../viewModels/SheepVM";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import styles from '../../assets/css/SheepList.module.css';

interface ISheepList {
    id: number;
}

export const SheepList: React.FC<ISheepList> = ({id}) => {
    const sheepService = new SheepService();
    const [sheepVMList, setSheepVMList] = useState<SheepVM[]>([]);
    const navigate = useNavigate();
    const {t} = useTranslation();
    
    useEffect(() => {
        const fetchSheep = async () => {
            try {
                const data = await sheepService.getAllVMsByShepherdId(id);
                setSheepVMList(data);
            } catch (error) {
                alert(error);
                setSheepVMList([]);
            }
        }
        fetchSheep();
    }, []);
    
    return (
        <div className={styles.list}>
            {sheepVMList.length > 0 ? (
                sheepVMList.map((sheep) => (
                    <div key={sheep.id} className={styles.card} onClick={() => navigate("sheep/" + sheep.id)}>
                        <div className={styles.cardHeader}>
                            <h2 className={styles.sheepId}>{t("sheepList.sheepHeader")} #{sheep.id}</h2>
                        </div>
                        <div className={styles.cardBody}>
                            <p><strong>{t("sheepList.breedHeader")}:</strong> {sheep.breed}</p>
                            <p><strong>{t("sheepList.sexHeader")}:</strong> {t(sheep.sex ? 'sheepList.maleText' : 'sheepList.femaleText')}</p>
                            <p><strong>{t("sheepList.lastFeedingDateHeader")}:</strong> {sheep.lastFeedingDate}</p>
                            <p><strong>{t("sheepList.lastShearingDateHeader")}:</strong> {sheep.lastShearingDate}</p>
                        </div>
                    </div>
                ))
            ) : (
                <p>{t("sheepList.notFoundHeader")}</p>
            )}
        </div>
    );
}
