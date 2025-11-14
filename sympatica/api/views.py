from datetime import datetime
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods
import json
from .models import Patient, Session, Reading


# ======================================================================
# ============================== PATIENTS ==============================
# ======================================================================
@require_http_methods(["GET"])
def patient_listall(request):
    """GET patients list"""
    patients = Patient.objects.all()

    data = [{
        'patient_id': str(p.patient_id),
        'first_name': p.first_name,
        'last_name': p.last_name,
        'full_name': p.full_name,
        'date_of_birth': p.date_of_birth.isoformat(),
        'age': p.age,
        'created_at': p.created_at.isoformat(),
    } for p in patients]

    return JsonResponse(data, safe=False)


@csrf_exempt
@require_http_methods(["POST"])
def patient_create(request):
    """POST create new patient"""
    try:
        body = json.loads(request.body)

        dob = datetime.strptime(body['date_of_birth'], '%Y-%m-%d').date()

        patient = Patient.objects.create(
            first_name=body['first_name'],
            last_name=body['last_name'],
            date_of_birth=dob,
        )

        return JsonResponse({
            'patient_id': str(patient.patient_id),
            'first_name': patient.first_name,
            'last_name': patient.last_name,
            'full_name': patient.full_name,
            'date_of_birth': patient.date_of_birth.isoformat(),
            'age': patient.age,
        }, status=201)

    except KeyError as e:
        return JsonResponse({'error': f'Missing field: {e}'}, status=400)
    except json.JSONDecodeError:
        return JsonResponse({'error': 'Invalid JSON'}, status=400)
    except ValueError:
        return JsonResponse({'error': 'Invalid date format. Use YYYY-MM-DD'}, status=400)


@require_http_methods(["GET"])
def patient_view(request, patient_id):
    """GET patient"""
    try:
        patient = Patient.objects.get(patient_id=patient_id)

        data = {
            'patient_id': str(patient.patient_id),
            'first_name': patient.first_name,
            'last_name': patient.last_name,
            'full_name': patient.full_name,
            'date_of_birth': patient.date_of_birth.isoformat(),
            'age': patient.age,
            'created_at': patient.created_at.isoformat(),
            'updated_at': patient.updated_at.isoformat(),
        }

        return JsonResponse(data)
    except Patient.DoesNotExist:
        return JsonResponse({'error': 'Patient not found'}, status=404)


@csrf_exempt
@require_http_methods(["PUT", "PATCH"])
def patient_edit(request, patient_id):
    """PUT/PATCH edit patient"""
    try:
        patient = Patient.objects.get(patient_id=patient_id)
        body = json.loads(request.body)

        if 'first_name' in body:
            patient.first_name = body['first_name']
        if 'last_name' in body:
            patient.last_name = body['last_name']
        if 'date_of_birth' in body:
            patient.date_of_birth = body['date_of_birth']

        patient.save()

        return JsonResponse({
            'patient_id': str(patient.patient_id),
            'first_name': patient.first_name,
            'last_name': patient.last_name,
            'full_name': patient.full_name,
            'date_of_birth': patient.date_of_birth.isoformat(),
        })
    except Patient.DoesNotExist:
        return JsonResponse({'error': 'Patient not found'}, status=404)
    except json.JSONDecodeError:
        return JsonResponse({'error': 'Invalid JSON'}, status=400)


@csrf_exempt
@require_http_methods(["DELETE"])
def patient_delete(request, patient_id):
    """DELETE patient (cascades to sessions and readings)"""
    try:
        patient = Patient.objects.get(patient_id=patient_id)
        patient.delete()

        return JsonResponse({'status': 'patient deleted', 'patient_id': str(patient_id)})
    except Patient.DoesNotExist:
        return JsonResponse({'error': 'Patient not found'}, status=404)


# ======================================================================
# ============================== SESSIONS ==============================
# ======================================================================
@require_http_methods(["GET"])
def session_listall(request):
    """GET all sessions"""
    patient_id = request.GET.get('patient_id')

    if patient_id:
        sessions = Session.objects.filter(patient__patient_id=patient_id)
    else:
        sessions = Session.objects.all()

    data = [{
        'session_id': str(s.session_id),
        'patient_id': str(s.patient.patient_id),
        'patient_name': s.patient.full_name,
        'session_name': s.session_name,
        'started_at': s.started_at.isoformat(),
        'ended_at': s.ended_at.isoformat() if s.ended_at else None,
        'duration_seconds': s.duration_seconds,
        'reading_count': s.reading_count,
    } for s in sessions]

    return JsonResponse(data, safe=False)


@require_http_methods(["GET"])
def session_view(request, session_id):
    """GET a specific session"""
    try:
        session = Session.objects.get(session_id=session_id)

        data = {
            'session_id': str(session.session_id),
            'patient_id': str(session.patient.patient_id),
            'patient_name': session.patient.full_name,
            'session_name': session.session_name,
            'notes': session.notes,
            'started_at': session.started_at.isoformat(),
            'ended_at': session.ended_at.isoformat() if session.ended_at else None,
            'duration_seconds': session.duration_seconds,
            'reading_count': session.reading_count,
        }

        return JsonResponse(data)
    except Session.DoesNotExist:
        return JsonResponse({'error': 'Session not found'}, status=404)


@csrf_exempt
@require_http_methods(["POST"])
def session_create(request):
    """POST create a new session"""
    try:
        body = json.loads(request.body)

        patient = Patient.objects.get(patient_id=body['patient_id'])

        session = Session.objects.create(
            patient=patient,
            session_name=body.get('session_name', ''),
            notes=body.get('notes', ''),
        )

        return JsonResponse({
            'session_id': str(session.session_id),
            'patient_id': str(session.patient.patient_id),
            'session_name': session.session_name,
            'started_at': session.started_at.isoformat(),
        }, status=201)

    except Patient.DoesNotExist:
        return JsonResponse({'error': 'Patient not found'}, status=404)
    except KeyError as e:
        return JsonResponse({'error': f'Missing field: {e}'}, status=400)
    except json.JSONDecodeError:
        return JsonResponse({'error': 'Invalid JSON'}, status=400)


@csrf_exempt
@require_http_methods(["PUT", "PATCH"])
def session_edit(request, session_id):
    """PUT/PATCH edit session"""
    try:
        session = Session.objects.get(session_id=session_id)
        body = json.loads(request.body)

        if 'session_name' in body:
            session.session_name = body['session_name']
        if 'notes' in body:
            session.notes = body['notes']

        session.save()

        return JsonResponse({
            'session_id': str(session.session_id),
            'session_name': session.session_name,
            'notes': session.notes,
        })
    except Session.DoesNotExist:
        return JsonResponse({'error': 'Session not found'}, status=404)
    except json.JSONDecodeError:
        return JsonResponse({'error': 'Invalid JSON'}, status=400)


@csrf_exempt
@require_http_methods(["DELETE"])
def session_delete(request, session_id):
    """DELETE session (cascades to readings)"""
    try:
        session = Session.objects.get(session_id=session_id)
        session.delete()

        return JsonResponse({'status': 'session deleted', 'session_id': str(session_id)})
    except Session.DoesNotExist:
        return JsonResponse({'error': 'Session not found'}, status=404)


@csrf_exempt
@require_http_methods(["POST"])
def session_end(request, session_id):
    """POST end a session"""
    try:
        session = Session.objects.get(session_id=session_id)
        session.end_session()

        return JsonResponse({
            'status': 'session ended',
            'session_id': str(session.session_id),
            'ended_at': session.ended_at.isoformat(),
            'duration_seconds': session.duration_seconds,
        })
    except Session.DoesNotExist:
        return JsonResponse({'error': 'Session not found'}, status=404)


@require_http_methods(["GET"])
def session_readings(request, session_id):
    """GET all readings for a session"""
    try:
        session = Session.objects.get(session_id=session_id)
        readings = Reading.objects.filter(session=session).order_by('timestamp')

        # Group by type
        data = {
            'bvp': [],
            'eda': [],
            'temperature': [],
            'acc': []
        }

        for reading in readings:
            reading_data = {
                'timestamp': reading.timestamp.isoformat(),
                'value': reading.value
            }

            if reading.reading_type in data:
                data[reading.reading_type].append(reading_data)

        return JsonResponse({
            'session_id': str(session.session_id),
            'patient_name': session.patient.full_name,
            'session_name': session.session_name,
            'readings': data
        })

    except Session.DoesNotExist:
        return JsonResponse({'error': 'Session not found'}, status=404)


# ======================================================================
# ============================== READINGS ==============================
# ======================================================================
@require_http_methods(["GET"])
def reading_listall(request):
    """GET readings"""
    session_id = request.GET.get('session_id')

    if session_id:
        readings = Reading.objects.filter(session__session_id=session_id)
    else:
        readings = Reading.objects.all()[:100]

    data = [{
        'id': r.id,
        'session_id': str(r.session.session_id),
        'reading_type': r.reading_type,
        'value': r.value,
        'timestamp': r.timestamp.isoformat(),
    } for r in readings]

    return JsonResponse(data, safe=False)


@csrf_exempt
@require_http_methods(["POST"])
def reading_create(request):
    """POST create multiple readings in batch"""
    try:
        body = json.loads(request.body)
        readings_data = body.get('readings', [])

        if not readings_data:
            return JsonResponse({'error': 'No readings provided'}, status=400)

        # Prepare all readings for bulk insert
        readings_to_create = []
        for reading_data in readings_data:
            session = Session.objects.get(session_id=reading_data['session_id'])

            readings_to_create.append(Reading(
                session=session,
                reading_type=reading_data['reading_type'],
                value=str(reading_data.get('value')),
            ))

        # Bulk create all at once - MUCH faster!
        created_readings = Reading.objects.bulk_create(readings_to_create)

        return JsonResponse({
            'created': len(created_readings),
            'message': f'Successfully created {len(created_readings)} readings'
        }, status=201)

    except Session.DoesNotExist:
        return JsonResponse({'error': 'Session not found'}, status=404)
    except KeyError as e:
        return JsonResponse({'error': f'Missing field: {e}'}, status=400)
    except json.JSONDecodeError:
        return JsonResponse({'error': 'Invalid JSON'}, status=400)
