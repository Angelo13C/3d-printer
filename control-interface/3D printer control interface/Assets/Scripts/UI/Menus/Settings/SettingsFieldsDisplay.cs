using TMPro;
using UnityEngine;

namespace UI.Menus.Settings
{
    public class SettingsFieldsDisplay : MonoBehaviour
    {
        [SerializeField] private SettingsFields _settingsFields;
        
        [Header("Prefabs")]
        [SerializeField] private GameObject _blockPrefab;
        [SerializeField] private GameObject _fieldPrefab;

        [Header("Customization")] 
        [SerializeField] private float _rightPaddingForUnits = 20;
        
        private void Awake()
        {
            foreach (var block in _settingsFields.Blocks)
            {
                var blockGameObject = Instantiate(_blockPrefab, transform);
                blockGameObject.GetComponent<TextMeshProUGUI>().text = block.Title;

                foreach (var field in block.Fields)
                {
                    var fieldGameObject = Instantiate(_fieldPrefab, blockGameObject.transform);
                    fieldGameObject.GetComponentInChildren<TextMeshProUGUI>().text = field.Name;

                    var inputField = fieldGameObject.GetComponentInChildren<TMP_InputField>();
                    
                    ApplyFieldTypeSettings(field.Type, inputField);
                }
            }
        }

        private void ApplyFieldTypeSettings(SettingsFields.Block.Field.FieldType fieldType, TMP_InputField inputField)
        {
            switch (fieldType)
            {
                case SettingsFields.Block.Field.FieldType.Int:
                    inputField.contentType = TMP_InputField.ContentType.IntegerNumber;
                    break;
                case SettingsFields.Block.Field.FieldType.Distance:
                    inputField.contentType = TMP_InputField.ContentType.DecimalNumber;

                    var indexOfUnits = 0;
                    var width = 0f;
                    for (; indexOfUnits < inputField.transform.childCount; indexOfUnits++)
                    {
                        var unitsText = inputField.transform.GetChild(indexOfUnits).GetComponent<TextMeshProUGUI>();
                        if (unitsText != null)
                        {
                            unitsText.text = "mm";
                            unitsText.ForceMeshUpdate();
                            width = unitsText.textBounds.size.x;
                            break;
                        }
                    }
                    
                    var indexOfTextArea = indexOfUnits == 0 ? 1 : 0;
                    var textAreaTransform = inputField.transform.GetChild(indexOfTextArea).GetComponent<RectTransform>();
                    textAreaTransform.offsetMax = new Vector2(-(width + _rightPaddingForUnits), textAreaTransform.offsetMax.y);
                    
                    break;
            }
        }
    }
}