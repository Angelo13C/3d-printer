using UnityEngine;

namespace UI.Menus.Base
{
    public class MenuUI : MonoBehaviour
    {
        [SerializeField] private Canvas[] _menus;

        private Canvas _currentlySelectedMenu = null;
        
        private void Awake()
        {
            foreach (var menu in _menus)
                menu.gameObject.SetActive(false);
        }

        public void OpenMenu(Canvas menu)
        {
            if (_currentlySelectedMenu != null)
                _currentlySelectedMenu.gameObject.SetActive(false);

            _currentlySelectedMenu = menu;
            _currentlySelectedMenu.gameObject.SetActive(true);
        }
    }
}