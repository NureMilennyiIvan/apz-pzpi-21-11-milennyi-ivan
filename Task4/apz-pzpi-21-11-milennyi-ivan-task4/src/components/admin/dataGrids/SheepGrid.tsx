import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import styles from "../../../assets/css/GridComponent.module.css";
import { SheepService } from "../../../api/services/SheepService";
import { Sheep } from "../../../models/Sheep";

export const SheepGrid = () =>{
    const sheepService = new SheepService();
    const [selectedSheep, setSelectedSheep] = useState<Sheep | null>(null);
    const [sheep, setSheep] = useState<Sheep[]>([]);
    const [trigger, setTrigger] = useState<boolean>(true);

    const {t} = useTranslation();
    const navigate = useNavigate();
    
    useEffect(() => {
        const fetchShepherds = async () => {
          try {
            const data = await sheepService.getAll();
            setSheep(data);
          } catch (error) {
            alert(error);
            setSheep([]);
          }
        };
        fetchShepherds();
    }, [trigger]);

    const handleRowSelection = (sheep: Sheep) => {
        setSelectedSheep(sheep);
    };
    const createSheep = () => {
        navigate("/sheep/create")
    };
    const reassignShepherd = (sheepId: number) =>{
        navigate(`/sheep/reassign-shepherd/${sheepId}`)
    }
    const editSheep = (id: number) => {
        navigate("/sheep/edit/" + id);
    }
    const deleteSheep = async (id: number) => {
        try{
            await sheepService.delete(id);
            setSelectedSheep(null);
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
                        <th className={styles.th}>{t("sheepGrid.breedIdHeader")}</th>
                        <th className={styles.th}>{t("sheepGrid.shepherdIdHeader")}</th>
                        <th className={styles.th}>{t("sheepGrid.sexHeader")}</th>
                        <th className={styles.th}>{t("sheepGrid.birthDateHeader")}</th>
                        <th className={styles.th}>{t("sheepGrid.weightHeader")}</th>
                        <th className={styles.th}>{t("sheepGrid.temperatureScannerIdHeader")}</th>
                    </tr>
                </thead>
                <tbody>
                    {sheep.length != 0 ? (
                        sheep.map((sheep) => (
                            <tr key={sheep.id} 
                                className={`${styles.tr} ${selectedSheep?.id === sheep.id ? styles.selected : ''}`}>
                                <td className={styles.td}>
                                    <input
                                        type="radio"
                                        name="selectedUser"
                                        checked={selectedSheep?.id === sheep.id}
                                        onChange={() => handleRowSelection(sheep)}
                                    />
                                </td>
                                <td className={styles.td}>{sheep.id}</td>
                                <td className={styles.td}>{sheep.breed_id}</td>
                                <td className={styles.td}>{sheep.shepherd_id ? sheep.shepherd_id : "null"}</td>
                                <td className={styles.td}>{sheep.sex.toString()}</td>
                                <td className={styles.td}>{sheep.birth_date}</td>
                                <td className={styles.td}>{sheep.weight}</td>
                                <td className={styles.td}>{sheep.temperature_scanner_id ? sheep.temperature_scanner_id : "null"}</td>
                            </tr>
                        ))
                    ) : (
                        <div className={styles.error}>{t("sheepGrid.notFoundHeader")}</div>
                    )}
                </tbody>
            </table>
            <div className={styles.actionButtonsContainer}>
                <button className={styles.actionButton} onClick={() => createSheep()}>
                    {t("gridBase.addButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedSheep?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedSheep ? editSheep(selectedSheep.id!) : undefined}
                    style={{ cursor: selectedSheep ? 'pointer' : 'default' }}>
                    {t("gridBase.editButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedSheep?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedSheep ? reassignShepherd(selectedSheep.id!) : undefined}
                    style={{ cursor: selectedSheep ? 'pointer' : 'default' }}>
                    {t("sheepGrid.reassignShepherdButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedSheep?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedSheep ? deleteSheep(selectedSheep.id!) : undefined}
                    style={{ cursor: selectedSheep ? 'pointer' : 'default' }}>
                    {t("gridBase.deleteButtonText")}
                </button>
            </div>
        </div>
    );
}