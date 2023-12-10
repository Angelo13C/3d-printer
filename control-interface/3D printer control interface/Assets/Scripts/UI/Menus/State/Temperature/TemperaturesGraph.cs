using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI.Extensions;

namespace UI.Menus.State.Temperature
{
    public class TemperaturesGraph : MonoBehaviour
    {
        [SerializeField] private UILineRenderer _hotendLineRenderer;
        [SerializeField] private UILineRenderer _bedLineRenderer;
        
        [Header("Configuration")]
        [SerializeField] [Min(0)] private float _graphTimeInSeconds = 120;

        [Space]
        [SerializeField] private Vector2Int _temperatureRange = new(0, 300);

        private float _lastTemperaturePlotTime;

        public void PlotTemperatures(int hotendTemperature, int bedTemperature)
        {
            void PlotTemperature(int temperature, UILineRenderer lineRenderer)
            {
                // Y axis
                var temperaturePercentage = Mathf.InverseLerp(_temperatureRange.x, _temperatureRange.y, temperature);
                
                // X axis
                var plotDeltaTime = Time.time - _lastTemperaturePlotTime;
                var plotDeltaTimePercentage = plotDeltaTime / _graphTimeInSeconds;
                if (lineRenderer.Points[^1].x >= 0.999f)
                {
                    var newPoints = new List<Vector2>(lineRenderer.Points.Length + 1);
                    for (var i = 0; i < lineRenderer.Points.Length; i++)
                    {
                        var movedPoint = lineRenderer.Points[i];
                        movedPoint.x -= plotDeltaTimePercentage;
                        if (movedPoint.x <= 0)
                        {
                            var nextPoint = lineRenderer.Points[i + 1];
                            nextPoint.x -= plotDeltaTimePercentage;
                            if(nextPoint.x <= 0)
                                continue;
                            var t = Mathf.Abs(movedPoint.x) / plotDeltaTimePercentage;
                            movedPoint = Vector2.Lerp(movedPoint, nextPoint, t);
                        }

                        if (newPoints.Count > 0 && movedPoint.x < newPoints[^1].x + 0.005f)
                            newPoints[^1] = movedPoint;
                        else
                            newPoints.Add(movedPoint);
                    }
                    newPoints.Add(new Vector2(1, temperaturePercentage));

                    lineRenderer.Points = newPoints.ToArray();
                }
                else
                {
                    var newPoints = new Vector2[lineRenderer.Points.Length + 1];
                    Array.Copy(lineRenderer.Points, newPoints, lineRenderer.Points.Length);

                    var x = lineRenderer.Points.Length == 0 ? 0 : lineRenderer.Points[^1].x;
                    newPoints[^1] = new Vector2(x + plotDeltaTimePercentage, temperaturePercentage);

                    lineRenderer.Points = newPoints;
                }
            }
            
            PlotTemperature(hotendTemperature, _hotendLineRenderer);
            PlotTemperature(bedTemperature, _bedLineRenderer);

            _lastTemperaturePlotTime = Time.time;
        }
    }
}