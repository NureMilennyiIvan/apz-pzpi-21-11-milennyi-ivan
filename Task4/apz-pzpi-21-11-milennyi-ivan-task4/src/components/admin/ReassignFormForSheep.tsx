import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams } from "react-router-dom";
import { ShepherdService } from "../../api/services/ShepherdService";
import { SheepService } from "../../api/services/SheepService";
import styles from "../../assets/css/ReassignFormForSheep.module.css";
import { IUserProps } from "../properties/IUserProps";
import { useEffectUser } from "../../utils/helpers";
import { TemperatureScannerService } from "../../api/services/TemperatureScannerService";

interface FormProps extends IUserProps {
  entityType: "Shepherd" | "TemperatureScanner";
}

export const ReassignFormForSheep: React.FC<FormProps> = ({user, entityType}) => {
  const [entitiesList, setEntitiesList] = useState<any[]>([]);
  const [selectedEntityId, setSelectedEntityId] = useState<number | null>(null);
  const sheepId = parseInt(useParams().sheepId!);
  const navigate = useNavigate();
  const { t } = useTranslation();

  const services = {
    Shepherd: new ShepherdService(),
    TemperatureScanner: new TemperatureScannerService(),
  };

  useEffectUser(user, navigate);

  const getEntitiesPromise = (): Promise<any> => {
    switch (entityType) {
      case "Shepherd":
        return services.Shepherd.getAllVMs();
      case "TemperatureScanner":
        return services.TemperatureScanner.getAllUnassignedScannersIds();
    }
  }
  useEffect(() => {
    const fetchShepherds = async () => {
      try {
        const data = await getEntitiesPromise();
        setEntitiesList(data);
      } catch (error) {
        console.error(error);
        setEntitiesList([]);
      }
    };

    fetchShepherds();
  }, []);

  const handleSubmit = async () => {
    const sheepService = new SheepService();
    try {
      switch (entityType) {
        case "Shepherd":
          await sheepService.changeShepherd(sheepId, selectedEntityId);
          break;
        case "TemperatureScanner":
          await sheepService.changeTemperatureScanner(sheepId, selectedEntityId);
          break;
      }
      navigate(-1);
    } catch (error) {
      console.error(error);
    }
  };

  const renderFormFields = () => {
    switch (entityType) {
      case "Shepherd":
        return(<>
          <label className={styles.inputLabel}>{t("reassignFormForSheep.shepherdsHeader")}</label>
          <select
            className={styles.select}
            value={selectedEntityId ?? ""}
            onChange={(e) => setSelectedEntityId(e.target.value === "" ? null : parseInt(e.target.value))}
          >
            <option value="">{t("reassignFormForSheep.dontAssignOption")}</option>
            {entitiesList.map((entity) => (
              <option key={entity.id} value={entity.id!}>
                {entity.name} {entity.surname}
              </option>
            ))}
          </select>
        </>)
      case "TemperatureScanner":
        return(<>
          <label className={styles.inputLabel}>{t("reassignFormForSheep.temperatureScannersHeader")}</label>
          <select
            className={styles.select}
            value={selectedEntityId ?? ""}
            onChange={(e) => setSelectedEntityId(e.target.value === "" ? null : parseInt(e.target.value))}
          >
            <option value="">{t("reassignFormForSheep.dontAssignOption")}</option>
            {entitiesList.map((id) => (
              <option key={id} value={id!}>
                {`${t("reassignFormForSheep.scannerHeader")}${id}`}
              </option>
            ))}
          </select>
        </>)
    }
  }

  return (
    <div className={styles.container}>
      <div className={styles.form}>
        <div className={styles.labelContainer}>
          <label className={styles.label}>{`${t("reassignFormForSheep.sheepHeader")}${sheepId}`}</label>
        </div>
        <div className={styles.inputContainer}>
          {renderFormFields()}
        </div>
        <button className={styles.button} onClick={handleSubmit}>
          {t("reassignFormForSheep.assignTextButton")}
        </button>
      </div>
    </div>
  );
};

