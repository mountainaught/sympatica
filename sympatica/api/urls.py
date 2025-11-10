"""
URL configuration for backend project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/stable/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.urls import path
from . import views

urlpatterns = [
    # Patients
    path('patients/', views.patient_listall, name='patients-list'),
    path('patients/create', views.patient_create, name='patient-create'),
    path('patients/<uuid:patient_id>/', views.patient_view, name='patient-view'),
    path('patients/<uuid:patient_id>/edit', views.patient_edit, name='patient-edit'),
    path('patients/<uuid:patient_id>/delete', views.patient_delete, name='patient-delete'),

    # Sessions
    path('sessions/', views.session_listall, name='session-list'),
    path('sessions/create', views.session_create, name='session-create'),
    path('sessions/<int:session_id>/', views.session_view, name='session-view'),
    path('sessions/<int:session_id>/edit', views.session_edit, name='session-edit'),
    path('sessions/<int:session_id>/delete', views.session_delete, name='session-delete'),

    # Readings
    path('readings/', views.reading_listall, name='readings-list'),
    path('readings/create/', views.reading_create, name='reading-create'),
]
