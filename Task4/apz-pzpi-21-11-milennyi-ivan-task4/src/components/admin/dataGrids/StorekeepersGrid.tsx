import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import styles from "../../../assets/css/GridComponent.module.css"
import { StorekeeperService } from "../../../api/services/StorekeeperService";
import { Storekeeper } from "../../../models/Storekeeper";

export const StorekeepersGrid = () =>{
    const storekeeperService = new StorekeeperService();
    const [selectedStorekeeper, setSelectedStorekeeper] = useState<Storekeeper | null>(null);
    const [storekeepers, setStorekeepers] = useState<Storekeeper[]>([]);
    const [trigger, setTrigger] = useState<boolean>(true);

    const {t} = useTranslation();
    const navigate = useNavigate();
    
    useEffect(() => {
        const fetchStorekeepers = async () => {
          try {
            const data = await storekeeperService.getAll();
            setStorekeepers(data);
          } catch (error) {
            alert(error);
            setStorekeepers([]);
          }
        };
        fetchStorekeepers();
    }, [trigger]);

    const handleRowSelection = (storekeeper: Storekeeper) => {
        setSelectedStorekeeper(storekeeper);
    };
    
    const createStorekeeper = () => {
        navigate("/storekeeper/create");
    }
    const editStorekeeper = (id: number) => {
        navigate("/storekeeper/edit/" + id);
    }
    const deleteStorekeeper = async (id: number) => {
        try{
            await storekeeperService.delete(id);
            setSelectedStorekeeper(null);
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
                        <th className={styles.th}>{t("storkeepersGrid.usernameHeader")}</th>
                        <th className={styles.th}>{t("storkeepersGrid.passwordHeader")}</th>
                        <th className={styles.th}>{t("storkeepersGrid.nameHeader")}</th>
                        <th className={styles.th}>{t("storkeepersGrid.surnameHeader")}</th>
                    </tr>
                </thead>
                <tbody>
                    {storekeepers.length != 0 ? (
                        storekeepers.map((storekeeper) => (
                            <tr key={storekeeper.id} 
                                className={`${styles.tr} ${selectedStorekeeper?.id === storekeeper.id ? styles.selected : ''}`}>
                                <td className={styles.td}>
                                    <input
                                        type="radio"
                                        name="selectedUser"
                                        checked={selectedStorekeeper?.id === storekeeper.id}
                                        onChange={() => handleRowSelection(storekeeper)}
                                    />
                                </td>
                                <td className={styles.td}>{storekeeper.id}</td>
                                <td className={styles.td}>{storekeeper.username}</td>
                                <td className={styles.td}>{storekeeper.password}</td>
                                <td className={styles.td}>{storekeeper.name}</td>
                                <td className={styles.td}>{storekeeper.surname}</td>
                            </tr>
                        ))
                    ) : (
                        <div className={styles.error}>{t("storkeepersGrid.notFoundHeader")}</div>
                    )}
                </tbody>
            </table>
            <div className={styles.actionButtonsContainer}>
                <button className={styles.actionButton} onClick={() => createStorekeeper()}>
                    {t("gridBase.addButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedStorekeeper?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedStorekeeper ? editStorekeeper(selectedStorekeeper.id!) : undefined}
                    style={{ cursor: selectedStorekeeper ? 'pointer' : 'default' }}>
                    {t("gridBase.editButtonText")}
                </button>
                <button 
                    className={`${styles.actionButton} ${selectedStorekeeper?.id ? '' : styles.disabledButton}`}
                    onClick={() => selectedStorekeeper ? deleteStorekeeper(selectedStorekeeper.id!) : undefined}
                    style={{ cursor: selectedStorekeeper ? 'pointer' : 'default' }}>
                    {t("gridBase.deleteButtonText")}
                </button>
            </div>
        </div>
    );
}