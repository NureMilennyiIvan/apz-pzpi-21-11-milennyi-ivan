import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { ShepherdService } from "../../api/services/ShepherdService";
import { SheepService } from "../../api/services/SheepService";
import styles from "../../assets/css/ReassignShepherdForm.module.css";
import { ShepherdVM } from "../../viewModels/ShepherdVM";
import { IUserProps } from "../properties/IUserProps";
import { useEffectUser } from "../../utils/helpers";

export const ReassignShepherdForm: React.FC<IUserProps> = ({user}) => {
  const [shepherdVMs, setShepherdVMs] = useState<ShepherdVM[]>([]);
  const [selectedShepherdId, setSelectedShepherdId] = useState<number | null>(null);
  const sheepId = parseInt(useParams().sheepId!);
  const navigate = useNavigate();
  const { t } = useTranslation();

  useEffectUser(user, navigate);
  useEffect(() => {
    const fetchShepherds = async () => {
      const shepherdService = new ShepherdService();
      try {
        const data = await shepherdService.getAllVMs();
        setShepherdVMs(data);
      } catch (error) {
        console.error(error);
        setShepherdVMs([]);
      }
    };

    fetchShepherds();
  }, []);

  const handleSubmit = async () => {
    const sheepService = new SheepService();
    try {
      await sheepService.changeShepherd(sheepId, selectedShepherdId);
      navigate(-1);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.form}>
        <div className={styles.labelContainer}>
          <label className={styles.label}>{`${t("reassignShepherdForm.sheepHeader")}${sheepId}`}</label>
        </div>
        <div className={styles.inputContainer}>
          <label className={styles.inputLabel}>{t("reassignShepherdForm.shepherdIdHeader")}</label>
          <select
            className={styles.select}
            value={selectedShepherdId ?? ""}
            onChange={(e) =>
              setSelectedShepherdId(e.target.value === "" ? null : parseInt(e.target.value))
            }
          >
            <option value="">{t("reassignShepherdForm.dontAssignOption")}</option>
            {shepherdVMs.map((shepherdVM) => (
              <option key={shepherdVM.id} value={shepherdVM.id!}>
                {shepherdVM.name} {shepherdVM.surname}
              </option>
            ))}
          </select>
        </div>
        <button className={styles.button} onClick={handleSubmit}>
          {t("reassignShepherdForm.assignTextButton")}
        </button>
      </div>
    </div>
  );
};

